# bevy-medieval-game
## important:
hi
## How to work with Cargo (Rust project manager)
- $ cargo run
  - Runs the project (very useful)
## How to work with git:
- $ git clone \<url\>
  - Downloads the repo (run only in the beginning)
- $ git pull
  - Pulls all the changes that have been made on the remote repo to your local one, updating yours to date. 
- $ git commit -a -m "\<your commit message\>"
  - "-a" adds all the files into a 'commit', which basically contains the changes you've made in your local repo. "-m" prompts you to enter a message for the commit. 
- $ git add \<file\>
  - If you wanna add only a single file or directory to your future commit (doesn't create a commit). 
- $ git push
  - Push the changes you've made in your commit to the remote repo.
### I haven't learned how pull requests work, which might be a much much much more reliable method...
## Special cases to git:
- When you want to undo "git add" and "git commit", bringing you to where you started off:
  - $ git reset HEAD~1
- When the remote repo is different and you want to overwrite that one with yours (be careful):
  - $ git push origin main --force
