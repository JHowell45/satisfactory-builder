set shell := ["powershell.exe", "-c"]

[private]
default:
    @just --list

# Git:

alias gc := git-commit
alias gp := git-push
alias gcp := git-commit-push

[group("git")]
git-commit msg:
    git add *
    git commit -m "{{ msg }}"

[group("git")]
git-push:
    git push

[group("git")]
git-commit-push msg:
    git add *
    git commit -m "{{ msg }}"
    git push
