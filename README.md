# Taskwarrior test data for taskwarrior-tui

Please provide a minimal working example of the bug with screenshots if possible.
If not possible, please provide a anonymized version of your `.taskrc` file and the output of `task next` (or whatever relevant taskwarrior feature you are using).

You can set the TASKDATA and TASKRC environment variables to point to a different location for temporary fresh taskwarrior session.

You can use the following fake task list to reproduce your error:

```bash
git clone https://github.com/kdheepak/taskwarrior-testdata/
```

After you clone the above repository, run the following lines in your shell.

```bash
export TASKRC=`pwd`/taskwarrior-testdata/.taskrc
export TASKDATA=`pwd`/taskwarrior-testdata/.task
```

Then run the following:

```bash
task import `pwd`/taskwarrior-testdata/export.json
```

See taskwarrior documentation for more information.

Use your favorite tool to generate a screenshot or a gif of the error.
