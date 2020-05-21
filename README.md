# Backup Analyzer
Compare filesets between two target directories. E.g. old backup and new. Find out which files are new and which were removed between them. Also discover duplicate files. 

This utility operates on sets of files ignoring directory structure and symlinks.

## Build
```shell
cargo build
```

## How to use
```shell
$ backup_diff --help
USAGE:
    backup_diff [FLAGS] <DIR_A> <DIR_B>

ARGS:
    <DIR_A>    First directory, e.g. newer version
    <DIR_B>    Second directory, e.g. older version

FLAGS:
    -l, --linear     Disable concurrent processing
    -h, --help       Prints help information
    -V, --version    Prints version information
```

## Example
```shell
$ backup_diff ./backup_07.07.2020 ./backup_01.01.2019
Listing directory [./backup_07.07.2020] in progress...Done
Concurrent Processing Enabled
Hashing 6 file(s), this may take long time.
Done
Listing directory [./backup_01.01.2019] in progress...Done
Concurrent Processing Enabled
Hashing 6 file(s), this may take long time.
Done

New files (1 items):
  ./backup_07.07.2020/a.txt

Removed files (1 items):
  ./backup_01.01.2019/dir3/c1.txt

Duplicates in `new` folder:
  [./backup_07.07.2020/a.txt]
    ./backup_07.07.2020/dir1/a1.txt (duplicate)
    ./backup_07.07.2020/dir1/dir2/a2.txt (duplicate)
  [./backup_07.07.2020/b.txt]
    ./backup_07.07.2020/dir1/b1.txt (duplicate)
    ./backup_07.07.2020/dir1/dir2/b2.txt (duplicate)

Duplicates in `old` folder:
  [./backup_01.01.2019/dir3/c1.txt]
    ./backup_01.01.2019/dir3/dir4/c2.txt (duplicate)
    ./backup_01.01.2019/c.txt (duplicate)
  [./backup_01.01.2019/dir3/b1.txt]
    ./backup_01.01.2019/dir3/dir4/b2.txt (duplicate)
    ./backup_01.01.2019/b.txt (duplicate)

Errors:
  <Empty>
```

## License
This utility is a free software and is licensed under GPLv3.