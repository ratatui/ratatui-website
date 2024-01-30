So you’ve decided to become a contributor (Thank you!), and it is time to make a pull request. In this guide, we're going to cover some of the ``git`` commands that can be useful at this stage of the process, especially if you run into any issues.

## The Basic Workflow

If you are completely new to git and/or open source projects, you may find this guide useful: [fork-commit-merge](https://github.com/fork-commit-merge/fork-commit-merge/blob/main/README.md). It will take you step-by-step through the basic open source workflow. 

You can also go in-depth into ``git`` with the [official documentation](https://git-scm.com/book/en/v2) and the [official reference manual](https://git-scm.com/docs).

## git rebase

As you contribute to Ratatui, you may run into conflicts between your PR or branch and the upstream code. This usually happens when your commit is made on an older version of the code base, and new changes have been implemented by other contributors.

With the ``git rebase`` command you can move your branch along the history of the repo and change the commit it is based on. This will allow you to base your branch on the latest commit. The command for this is:

```git
git rebase origin/main
```

It's very straightforward in theory. However, you may run into conflicts that have to be resolved before you can rebase your branch. One of the main causes for rebase conflicts is the presence of many small commits in your branch history. This is where squashing commits is useful.

To squash commits, you will have to run an interactive rebase using the ``-i`` flag.

```git
git rebase -i HEAD~x
```

``HEAD`` refers to the latest commit of the main branch, and ``x`` represents the number of commits that are on the branch. To find out what this number is you will need to run:

```git
git log --branches[=<branch>]
```

The ``--branches`` flag is useful when you’re working on a project that has a lot of commits. It allows you to check only the commits made on your branch.

Now when you run the interactive rebase command, ``git`` will return a file that contains all of your commits. 

In this file, you will see all of the commits, with the word “pick” in front of them. To combine the commits, change all the “pick” words, except for the first one, to “squash”.

After you save and close the file, a new one will open with all of the combined commit messages. All you have to do here is write the new commit message. This will squash your commits and generate a new commit message.

For more in-depth info on the ``rebase`` command, here is a link to [its entry in the reference guide](https://git-scm.com/book/en/v2/Git-Branching-Rebasing). You can also check out this [tutorial](https://www.atlassian.com/git/tutorials/rewriting-history/git-rebase) for some extra help.

## git commit

Another way to avoid many small commits that cause conflicts is through the ``git commit --amend`` command. For example, if you make a small change to a file or a mistake in the commit message, you can use ``--amend`` for a quick fix.

To change the commit message using ``--amend``, all you need to do is add the ``-m`` flag:

```git
git commit --amend -m "updated commit message”
```

To add an extra file to your previous commit, you will first need to stage it and then simply run the ``git commit --amend`` command. You can use the ``--no-edit`` flag to amend the commit without modifying the commit message.

```git
git add newfile.py
git commit --amend --no-edit
```

Be careful when using ``--amend`` because it actually creates an entirely new commit, which replaces the one you intend to amend. This can have a confusing effect when used on public commits, since it will modify the branch’s history. Only use ``--amend`` on branches that you are working on by yourself.

You may also run into issues with some pre-commit or commit message hooks. These hooks often run linting, tests, and other types of checks. To skip these tests, you can use the ``--no-verify`` flag, or its shorthand ``-n``.

```git
git commit --no-verify -m “message”
```

Learn more about ``commit`` [here](https://git-scm.com/docs/git-commit).

## git push

The ``--no-verify`` flag is also available for the ``git-push`` command, and it serves the same function. If you are running into pre-commit hooks when pushing a branch, try using this flag.

### Solving upstream issues

When you fork a repository, you can avoid a lot of issues by using the following command:

```git
git remote add upstream https://github.com/name-of-the-original-repo.git
```

This command adds the original repo as a reference to your fork. You can then use ``git pull`` to update your fork. This is useful when working on a project that receives daily or multiple updates on a regular basis. However, for shorter projects, you do not really need to add the upstream.

The problem is that you may run into issues when pushing, and in this situation, there is a very handy solution - the ``-u`` flag for the ``push`` command. This flag will automatically add the upstream to your repo and go through with the push. In our project, this may be an issue when we are running ``cargo-make`` to build the website.

If you run into upstream issues when pushing, try the following command:

```git
git push -u origin your-branch
```

### The ``-f`` flag

It's important to mention that if you run into any issues with your PR, you should not use ``git push -f`` in order to solve them. 

The ``-f`` flag is shorthand for ``--force`` and it rewrites the history of the repo, by removing any commits that are not found on your local machine. If you try to merge a repo that has been pushed with this flag, you will get a lot of unresolvable conflicts.

Find more info about ``push`` [here](https://git-scm.com/docs/git-push).

## Signing commits

With git, you can use any username and email address you want to make a commit, even a name and email that does not belong to you! In small teams, or closed projects, this is not an issue. However, in open source work, this can create confusion.

Thankfully, there are several ways to sign your commits and confirm your identity. We require commits to be signed for our repo, because it helps us improve the security of the project.

To find out more about how to sign your commits, check out the [official GitHub documentation on the topic](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits).

