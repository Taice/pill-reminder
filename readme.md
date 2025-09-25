# Pill reminder
the purpose of this app is because i was forgetting to take my pills so i made a cli thing to run with your shell that just reminds you when to take your pills.

# usage:
```
./pill-reminder --add {pill-name}:{hour}:{min}
```

this will make the program remind you about pill-name when the current time is more than {hour}:{min}
running just the binary `./pill-reminder` will print any due pills. when you take your pill, just do:

```
./pill-reminder --update {pill-name}
```

and it will update that pill so that when you run pill reminder today it won't warn you about that pill until the next day at {hour}:{min}
