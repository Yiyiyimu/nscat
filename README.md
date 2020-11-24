# nsexec
Enter certain **NS** and **EXEC**ute command in mount namespace.

For really slim container that even remove `cat`/`ls`, since nsenter to mount namespace could not find the command file in target ns,
this tool could achieve enter a namespace and print file / list directory.

### ls
Currently support `-l`, to also print symlink when needed.

### Example
```shell
$ sudo nsexec $PID cat $FILEPATH
$ sudo nsexec $PID ls -l $DIR
```
