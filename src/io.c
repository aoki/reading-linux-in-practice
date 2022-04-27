#define _GNU_SOURCE
#include <err.h>
#include <errno.h>
#include <fcntl.h>
#include <linux/fs.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

#define PART_SIZE (1024 * 1024 * 1024) // 1GB
#define ACCESS_SIZE (64 * 1024 * 1024) // 64MB

static char *progname;

// # 測定内容
// - I/O サイズによる性能の変化
// - シーケンシャルアクセスとランダムアクセスの違い
//
// # プログラム仕様
// - 指定したパーティションの先頭から1Gバイトまでの領域内に、合計64MバイトのI/Oを発行する
// - 読み書きの種類、アクセスパターン（シーケンシャルアクセス、ランダムアクセス）、及び1回あたりのI/Oサイズを指定できる
// - 受け取る引数
//     - 第1引数: ファイル名
//     - 第2引数: 本章の後半において説明する、カーネルによるI/O支援機能を有効にするかどうか（on, off）
//     - 第3引数: 読み書きの種類（r = 読み出し、 w = 書き込み）
//     - 第4引数: アクセスパターン（sec = シーケンシャルアクセス、 rand = ランダムアクセス）
//     - 第5引数: 1回あたりのI/Oサイズ（Kバイト）
int main(int argc, char *argv[]) {

    // 引数の数をチェック
    progname = argv[0];
    if (argc != 6) {
        fprintf(stderr, "usage: %s <filename> <kernel's help> <r/w> <access pattern> <block size[KB]>\n", progname);
        exit(EXIT_FAILURE);
    }

    // ファイル名を取得
    char *filename = argv[1];

    // カーネルによるI/O支援機能フラグを取得
    bool help;
    if (!strcmp(argv[2], "on")) {
        help = true;
    } else if (!strcmp(argv[2], "off")) {
        help = false;
    } else {
        fprintf(stderr, "kernel's help should be 'on' or 'off': %s\n", argv[2]);
        exit(EXIT_FAILURE);
    }

    // 読み書きの種類を取得
    int write_flag;
    if (!strcmp(argv[3], "r")) {
        write_flag = false;
    } else if (!strcmp(argv[3], "w")) {
        write_flag = true;
    } else {
        fprintf(stderr, "r/w should be 'r' or 'w': %s\n", argv[3]);
        exit(EXIT_FAILURE);
    }

    // ランダムアクセスかシーケンシャルアクセスかを取得
    bool random;
    if (!strcmp(argv[4], "seq")) {
        random = false;
    } else if (!strcmp(argv[4], "rand")) {
        random = true;
    } else {
        fprintf(stderr, "access pattern should be 'seq' or 'rand': %s\n", argv[4]);
        exit(EXIT_FAILURE);
    }

    // 1アクセスあたりのサイズを取得（KB）
    int part_size = PART_SIZE;
    int access_size = ACCESS_SIZE;
    int block_size = atoi(argv[5]) * 1024;
    if (block_size == 0) {
        fprintf(stderr, "block size should be > 0: %s\n", argv[5]);
        exit(EXIT_FAILURE);
    }
    if (access_size % block_size != 0) {
        fprintf(stderr, "access size(%d) should be multiple of block size: %s\n", access_size, argv[5]);
        exit(EXIT_FAILURE);
    }

    int maxcount = part_size / block_size;
    int count = access_size / block_size;
    int *offset = malloc(maxcount * sizeof(int));
    if (offset == NULL) {
        err(EXIT_FAILURE, "malloc() failed");
    }

    // `O_DIRECT` フラグを与えることで、ダイレクトI/Oを使う
    int flag = O_RDWR | O_EXCL;
    if (!help) {
        flag |= O_DIRECT;
    }

    int fd;
    fd = open(filename, flag);
    if (fd == -1) {
        err(EXIT_FAILURE, "open() failed");
    }

    int i;
    for (i = 0; i < maxcount; i++) {
        offset[i] = i;
    }
    if (random) {
        for (i = 0; i < maxcount; i++) {
            int j = rand() % maxcount;
            int tmp = offset[j];
            offset[i] = offset[j];
            offset[j] = tmp;
        }
    }

    // `ioctl()` によって、指定したデバイスに対応するストレージデバイスのセクタサイズを取得している
    // https://manpages.ubuntu.com/manpages/jammy/ja/man2/ioctl.2.html
    int sector_size;
    if (ioctl(fd, BLKSSZGET, &sector_size) == -1) {
        err(EXIT_FAILURE, "ioctl() failed");
    }

    char *buf;
    int e;
    // https://manpages.ubuntu.com/manpages/jammy/en/man3/posix_memalign.3posix.html
    // ストレージデバイスに受け渡しするデータを保持するバッファ用メモリ領域の確保には、 `malloc()` ではなく、
    // `posix_memalign()` を使う。獲得するメモリの先頭アドレスを指定した数の倍数にする（アライメントする）。
    // ダイレクト I/O に使用するバッファの先頭アドレスとサイズは、ストレージデバイスのセクタサイズの倍数になっている必要があるため。
    e = posix_memalign((void **)&buf, sector_size, block_size);
    if (e) {
        errno = e;
        err(EXIT_FAILURE, "posix_memalign() failed");
    }

    for (i = 0; i < count; i++) {
        ssize_t ret;
        if (lseek(fd, offset[i] * block_size, SEEK_SET) == -1) {
            err(EXIT_FAILURE, "lseek() failed");
        }
        if (write_flag) {
            ret = write(fd, buf, block_size);
            if (ret == -1) {
                err(EXIT_FAILURE, "write() failed");
            }
        } else {
            ret = read(fd, buf, block_size);
            if (ret == -1) {
                err(EXIT_FAILURE, "read() failed");
            }
        }

        // `fdatasync()` 関数によって、それより上の処理において発行している I/O の完了を待つ
        // ダイレクト I/O 出ない通常の I/O の場合、 `write()` 関数は I/O の発行依頼をするだけで完了を待たないため
        if (fdatasync(fd) == -1) {
            err(EXIT_FAILURE, "fdatasync() failed");
        }
        if (close(fd) == -1) {
            err(EXIT_FAILURE, "close() failed");
        }

        exit(EXIT_SUCCESS);
    }
}
