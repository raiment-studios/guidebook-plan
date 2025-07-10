#!/usr/bin/env bash

# Currently this assumes there will be no updates from outside the monorepo, which is true
# for now, but certainly not desirable long-term.  Any external updates will be
# overwritten (though the overwrites won't be lost in the revision history).
#
# There is certainly a better way to handle this. Perhaps snapshotting external repo
# after each publish, then creating using the git history of that snapshot as the 
# basis for a new branch of the repo, which can be merged into the remote.
# But that's a problem for another day...
#

GIT_URL=git@github.com:raiment-studios/guidebook-plan.git

echo "Publishing guidebook-plan to ${GIT_URL}"
rm -rf temp/
mkdir -p temp/

echo "Collecting files from the monorepo..."
git ls-files > temp/file_list.txt
tar -czf temp/files.tar.gz -T temp/file_list.txt


echo "Creating an empty repository with revision history..."
cd temp
git clone ${GIT_URL} guidebook-plan
mv guidebook-plan/.git temp-git
rm -rf guidebook-plan
mkdir guidebook-plan
mv temp-git guidebook-plan/.git

echo "Overlaying files from the monorepo..."
tar -xzf files.tar.gz -C guidebook-plan

echo "Committing changes..."
cd guidebook-plan
git add .
git commit -m "Publish commit from monorepo"
git push origin main

cd ../..
rm -rf temp/



