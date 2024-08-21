# Ratatui Examples

Note: Edits to the examples in this folder will be overwritten on each release. To update these
examples, do so in the main [Ratatui](https://github.com/ratatui/ratatui) repo.

This folder contains a copy of the examples from the latest release version of Ratatui. It is
generated using directly from <https://github.com/ratatui/ratatui/tree/latest/examples> using
git read-tree as suggested by a
[stackoverflow answer](https://stackoverflow.com/questions/23937436/add-subdirectory-of-remote-repo-with-git-subtree):

```shell
git remote add --fetch --track latest --no-tags ratatui https://github.com/ratatui/ratatui.git
git merge --strategy ours --no-commit ratatui/latest --allow-unrelated-histories --squash
git read-tree --prefix=code/ratatui-examples/examples/ -u ratatui/latest:examples
git commit -m 'Add ratatui examples'
```

To update the examples in the future:

```shell
# This first command should only need to be run once
git remote add --fetch --track latest --no-tags ratatui https://github.com/ratatui/ratatui.git

git merge --strategy ours --no-commit ratatui/latest --squash
git rm -rf code/ratatui-examples/examples
git read-tree --prefix=code/ratatui-examples/examples/ -u ratatui/latest:examples
git commit -m 'Update examples'
```
