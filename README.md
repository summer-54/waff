# tstcli
## Toaster command line interface

# Instance tree

```
|_ .
|_ .tost/
'|_ contest.json
 |_ tasks/
 '|_ A/
  '|_ info.json/
   |_ statements/
   '|_ ru.txt
    |_ en.tex
   ,|_ eo.md
   |_ samples/
   '|_ 1.in
    |_ 1.out
    |_ 2.in
   ,|_ 3.in
? ,|_ files/
  |_...

```

## .tost/contest.json

```
{
    id: "<contest_id>", // "{group_id}:{contest_id}" | "{contest_id}" -> group_id = -1
    tasks: [..., "<Litera>",...],
}
```

## .tost/tasks/{litera}/info.json

```
{
    name: "<task name : str>",
    litera: "<task Litera : str>",
    time_limit: <time limit : f32 [sec]>,
    memory_limit: <memory limit : u64 [byte]>,
}
```
