git add .
git commit -m "udpate:build linux"
git push 
git tag -d v1.1.4
git push origin -d v1.1.4

git tag v1.1.4
git push origin v1.1.4
