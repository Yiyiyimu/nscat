# nscat
Enter certain **NS** and **CAT** file contents

For really slim container that even remove `cat`, since nsenter to mount namespace could not find the command file in target ns, this tool could achieve enter a namespace and cat certain file.

### Example
```shell
$ sudo nscat /proc/$PID/ns/mnt $FILEPATH
```
