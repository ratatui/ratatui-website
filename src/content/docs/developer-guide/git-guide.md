---
title: Git guide
---

So you’ve decided to become a contributor (Thank you!), and you are having some trouble with `git`. In this guide, we're going to cover some of the `git` conventions and practices that we use in our repo.

## The Basic Workflow

If you are completely new to git and/or open source projects, you may find this guide useful: [fork-commit-merge](https://github.com/fork-commit-merge/fork-commit-merge/blob/main/README.md). It will take you step-by-step through the basic open source workflow. 

You can also go in-depth into `git` with the [official documentation](https://git-scm.com/book/en/v2) and the [official reference manual](https://git-scm.com/docs).

## Setting up images with Git LFS

Git LFS (Large File Storage) is a `git` extension that helps manage large files in a repository. 

‘git’ stores the entire history of a project, and when you clone a repository, you have to download every version of a file that is in that repo. As you can imagine, when you're dealing with large files that have been modified several times, the size of this history can get massive.

This is where Git LFS comes in. LFS has its own storage and reference system. When a file is added to LFS, it is replaced by a reference in the project files, and it is only downloaded when you run the `checkout` command to access a specific commit. This means that you only have to download one version of the large file, rather than its entire history.

We use Git LFS in our project to store images and other large files. To set up the repo, you will first have to [download and install Git LFS](https://git-lfs.com/). Alternatively, you can use a package manager. Then, run two commands:

```sh
git lfs install
git lfs pull
```

When you need to add images to the repo, the process is very similar to making normal commits in `git`. First add the images to the appropriate route in the filesystem. For example, if you were to add images to the page you are reading right now, you would go to the `website/src/content/docs/developer-guide/` folder. You would then have to run the following commands:

```sh
git add src/content/docs/developer-guide/picture.png
git commit -m “Added picture.png”
```

As you can see, you do not have to mention that this file has to be added to LFS. `git` will automatically detect the file extension, and if it is tracked by LFS, it will be handled by LFS.

## How to sign your commits

With git, you can use any username and email address you want to make a commit, even a name and email that does not belong to you! In small teams, or closed projects, this is not an issue. However, in open source work, it can create confusion.

Thankfully, there are several ways to sign your commits and confirm your identity. We require commits to be signed for our repo, because it helps us improve the security of the project.

To find out more about how to sign your commits, check out the [official GitHub documentation on the topic](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits).

## Skipping the pre-commit hooks

We use pre-commit and commit hooks to run linting, tests, and other types of checks. To skip these tests, you can use the `--no-verify` flag, or its shorthand `-n`.

```sh
git commit --no-verify -m “message”
```

The `--no-verify` flag is also available for the `git push` command, and it serves the same function. 

However, it's important to keep in mind that these tests are only skipped locally, and the flag is generally used to speed up development. Once you make a PR, you have to ensure that all the tests complete successfully.

## Good practices

### git rebase

As you contribute to open source projects, you may run into conflicts between your PR or branch and the upstream code. This usually happens when your commit is made on an older version of the code base, and new changes have been implemented by other contributors.

With the `git rebase` command you can move your branch along the history of the repo and change the commit it is based on. This will allow you to base your branch on the latest commit. The command for this is:

```sh
git rebase origin/main
```

It's important to note that `origin/main` has to be up to date with `ratatui/main`. This can be done in a number of ways, including by [setting `ratatui/main` as the upstream for your repo](#upstream), or through the GitHub interface.

![Updating a branch on Github](./update.png)

Rebasing is very straightforward in theory. However, you may run into conflicts that have to be resolved before you can rebase your branch. One of the main causes for rebase conflicts is the presence of many small commits in your branch history. This is where squashing commits is useful.

To squash commits, you will have to run an interactive rebase using the `-i` flag.

Make sure that you are on the right branch first, using:

```sh
git checkout your-branch
```

And then:

```sh
git rebase -i HEAD~x
```

In this scenario, `HEAD` refers to the latest commit of the main branch, and `x` represents the number of commits that are on the branch. To find out what this number is you will need to run:

```sh
git log --branches[=<branch>]
```

The `--branches` flag is useful when you’re working on a project that has a lot of commits. It allows you to check only the commits made on your branch.

Now when you run the interactive rebase command, `git` will return a file that contains all of your commits. 

```sh
pick f7bae90 commit-4
pick 6b70439 commit-5

# Rebase ff5e017..6b70439 onto ff5e017 (2 commands)
#
# Commands:
# p, pick <commit> = use commit
# r, reword <commit> = use commit, but edit the commit message
# e, edit <commit> = use commit, but stop for amending
# s, squash <commit> = use commit, but meld into previous commit
# f, fixup [-C | -c] <commit> = like "squash" but keep only the previous
#                    commit's log message, unless -C is used, in which case
#                    keep only this commit's message; -c is same as -C but
#                    opens the editor
```

In this file, you will see all of the commits, with the word “pick” in front of them. To combine the commits, change all the “pick” words, except for the first one, to “squash”.

After you save and close the file, a new one will open with all of the combined commit messages. All you have to do here is write the new commit message. This will squash your commits and generate a new commit message.

While this is a good coding practice in general that can help in many projects, for Ratatui, we’ve opted to automatically rebase and squash all commits when a PR is merged.

For more in-depth info on the `rebase` command, here is a link to [its entry in the reference guide](https://git-scm.com/book/en/v2/Git-Branching-Rebasing). You can also check out this [tutorial](https://www.atlassian.com/git/tutorials/rewriting-history/git-rebase) for some extra help.

### git commit

Another way to avoid many small commits that cause conflicts is through the `git commit --amend` command. For example, if you make a small change to a file or a mistake in the commit message, you can use `--amend` for a quick fix.

To change the commit message using `--amend`, all you need to do is add the `-m` flag:

```sh
git commit --amend -m "updated commit message”
```

To add an extra file to your previous commit, you will first need to stage it and then simply run the ``git commit --amend`` command. You can use the ``--no-edit`` flag to amend the commit without modifying the commit message.

```sh
git add newfile.rs
git commit --amend --no-edit
```

Be careful when using `--amend` because it actually creates an entirely new commit, which replaces the one you intend to amend. This can have a confusing effect when used on public commits, since it will modify the branch’s history. Only use `--amend` on branches that you are working on by yourself.

Learn more about `commit` [here](https://git-scm.com/docs/git-commit).

### git push

#### Solving upstream issues
<a name="upstream"></a>
When you fork a repository, you can avoid a lot of issues by using the following command:

```sh
git remote add upstream https://github.com/ratatui-org/ratatui.git
```

To keep your branch up to date with the changes in the main repo, simply run:

```sh
git fetch upstream
```

The `remote add upstream` command adds the original repo as a reference to your fork. This is useful when working on a project that receives daily or multiple updates on a regular basis. However, for shorter projects, you do not really need to add the upstream.

The problem is that you may run into issues when pushing, and in this situation, there is a very handy solution - the `-u` flag for the `push` command. This flag will automatically add the upstream to your repo and go through with the push. In our project, this may be an issue when we are running `cargo-make` to build the website.

If you run into upstream issues when pushing, try the following command:

```sh
git push -u origin your-branch
```

#### The `-f` flag

It's important to mention that if you run into any issues with your PR, you should not use `git push -f` in order to solve them. 

The `-f` flag is shorthand for `--force` and it rewrites the history of the repo, by removing any commits that are not found on your local machine. If you try to merge a repo that has been pushed with this flag, you will get a lot of unresolvable conflicts.

Find more info about `push` [here](https://git-scm.com/docs/git-push)

