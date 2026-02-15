# waff - Toaster command line interface

## Instance tree

```
|_ .
|_ .tost/
'|_ contest.json
 |_ tasks/
 '|_ A/
  '|_ info.json/
   |_ statements/
   '|_ ru.md (supported only it) 
    |_ en.tex 
   ,|_ eo.txt
   |_ samples/
   '|_ 1.in
    |_ 1.out
    |_ 2.in
   ,|_ 3.in
? ,|_ files/
  |_...

```

### .tost/contest.json

```
{
    id: "<contest_id>", /* "{group_id}:{contest_id}" | "{contest_id}" -> group_id = -1
    tasks: [..., "<Litera>",...],
}
```

### .tost/tasks/{litera}/info.json

```
{
    name: "<task name : str>",
    litera: "<task Litera : str>",
    time_limit: <time limit : f32 [sec]>,
    memory_limit: <memory limit : u64 [byte]>,
}
```
## waff_daemon

``` bash
TOKEN='<token>' ./client
```

>[!IMPORTANT]
> Use only single quotes `'`, to prevent replacing `$smth` substrings in token with enviroment variables.

>[!TIP]
> You can use `-z` to start background process

## waff

>[!WARNING]
> `waff_daemon` need to be already started

``` bash
waff <new | check | submit> ..
waff new [{group-id}:]{contest-id}
waff check {task-litera} {binary-file}
waff submit {task-litera} {code-file} [{language}]
```

# Compilation from source
 
``` bash
cargo build --release
cp target/release/{waff,waff-daemon} ~/.local/bin/
```

