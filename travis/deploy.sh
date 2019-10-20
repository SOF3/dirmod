#!/bin/bash

if [[ $TRAVIS_BRANCH == master ]] && [[ $RELEASE_FLAG == "--release" ]]; then
  git clone https://${GH_TOKEN}@github.com/SOF3/dirmod.git --branch=gh-pages $HOME/gh-pages
	[[ -d $HOME/gh-pages/latest ]] && rm -r $HOME/gh-pages/latest
  cp -r target/doc/ $HOME/gh-pages/latest
	if [[ $TRAVIS_TAG ]]; then
		[[ -d $HOME/gh-pages/$TRAVIS_TAG ]] && rm -r $HOME/gh-pages/$TRAVIS_TAG
		cp -r target/doc/ $HOME/gh-pages/$TRAVIS_TAG
		echo "<html><head><meta http-equiv='refresh' content='0; url=./${TRAVIS_TAG}/dirmod/'/></head><body>Redirecting...</body></html>" > $HOME/gh-pages/index.html
	fi
  cd $HOME/gh-pages
  git config user.name "Travis-CI: cargo doc"
  git config user.email "sofe2038@gmail.com"
  git add .
  git commit -m "Travis-CI doc build: $TRAVIS_COMMIT_MESSAGE"
  git push -u origin gh-pages
else
	echo "Only executing deployment on master with --release"
fi
