# minigrepf
This is the simple `minigrepf` command line tool from the book "The Rust Programming Language,"
chapter 12.  

Usage:
```
$ ls
poem.txt minigrepf

$ cat poem.txt
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

$ ./minigrepf to poem.txt
Are you nobody, too?
How dreary to be somebody!

$ IGNORE_CASE=1 ./minigrepf to poem.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
