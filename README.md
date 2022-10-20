# bevy-medieval-game
## important:
- hi
## How to work with Cargo (Rust project manager)
- $ cargo run
  - Runs the project (very useful)
## How to work with git:
- $ git clone \<url\>
  - Downloads the repo (run only in the beginning)
- $ git pull
  - Pulls all the changes that have been made on the remote repo to your local one, updating yours to date. (do this before editing any files to keep it up to date)
- $ git commit -a -m "\<your commit message\>"
  - "-a" adds all the files into a 'commit', which basically contains the changes you've made in your local repo. "-m" prompts you to enter a message for the commit. 
- $ git add \<file\>
  - If you wanna add only a single file or directory to your future commit (doesn't create a commit). 
- $ git commit -m "\<your commit message\>"
  - How to make a commit after using "git add" to individually add the files you want to push to the remote main branch.
- $ git push
  - Push the changes you've made in your commit to the remote repo.
## If there is a merge conflict when trying to git push:
- \<insert link\>
- If git push gives you:
~~~
 ! [rejected]        main -> main (fetch first)
error: failed to push some refs to 'https://github.com/senkowo/bevy-medieval-game.git'
hint: Updates were rejected because the remote contains work that you do
hint: not have locally. This is usually caused by another repository pushing
hint: to the same ref. You may want to first integrate the remote changes
hint: (e.g., 'git pull ...') before pushing again.
hint: See the 'Note about fast-forwards' in 'git push --help' for details.
~~~
  - Then it means the remote contains code that does not exist locally. Therefore, someone must've modified the remote within the last time you performed a git pull, so your version is behind. 
  - To fix this, do a git pull to bring your code up to date with the remote.
  - However, you will get this message:
~~~
hint: You have divergent branches and need to specify how to reconcile them.
hint: You can do so by running one of the following commands sometime before
hint: your next pull:
hint: 
hint:   git config pull.rebase false  # merge
hint:   git config pull.rebase true   # rebase
hint:   git config pull.ff only       # fast-forward only
hint: 
hint: You can replace "git config" with "git config --global" to set a default
hint: preference for all repositories. You can also pass --rebase, --no-rebase,
hint: or --ff-only on the command line to override the configured default per
hint: invocation.
~~~
  - The remote and local have differnt code, so you must merge the two together. 
  - I usually do git pull --rebase, which modifies your local to have the updated code of the remote. 
  - Then you finally do git push to update the remote with your changes, along with the previous latest remote code.
- $ git mergetool --tool=\<toolname\>
  - I use "git mergetool --tool=nvimdiff" since I have nvim installed. 
  - This will open up a big screen with many different tiles.
  - The large bottom one shows the summary.
  - The top left shows the local changes.
  - The top middle shows the staging?
  - The top right shows the state of the remote repo.
  - Modify the files accordingly to where there will be no conflicts when merging.
  - Once you are done, do git pull, and hope it works. 
  - If it does, do a git push and you did it yay.
## Special cases to git:
- When you want to undo "git add" and "git commit", bringing you to where you started off:
  - $ git reset HEAD~1
- When the remote repo is different and you want to overwrite that one with yours (be careful):
  - $ git push origin main --force
