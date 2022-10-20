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
## Special cases to git:
- When you want to undo "git add" and "git commit", bringing you to where you started off:
  - $ git reset HEAD~1
- When the remote repo is different and you want to overwrite that one with yours (be careful):
  - $ git push origin main --force
