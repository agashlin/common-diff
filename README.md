# common-diff

Diff groups of files

Example:

```
> common-diff group1-file1 group1-file2 group1-file3 -- group2-file1 group2-file2 group2-file3

       0x2: 14 13
     0x1f0: 00 02
```

Will find all bytes in common among the group1 files, and all bytes in common among the group2 files, and then will show all differences between these common bytes.

Major limitations:

- Only works with two groups.
- All comparisons assume the bytes don't shift.
- Only shows differences within the first 0x1000.
