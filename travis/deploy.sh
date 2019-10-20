#!/bin/bash

if [[ $TRAVIS_BRANCH == master ]] && [[ $RELASE_FLAG == "--release" ]]; then
  git clone https://${GH_TOKEN}@github.com/SOF3/dirmod.git --branch=gh-pages $HOME/gh-pages
  cp -r target/doc/* $HOME/gh-pages/latest
	if [[ $TRAVIS_TAG ]]; then
		cp -r target/doc/ $HOME/gh-pages/$TRAVIS_TAG
	fi
  cd $HOME/gh-pages
  git config user.name "Travis-CI: cargo doc"
  git config user.email "sofe2038@gmail.com"
  git add .
  git commit -m "Travis-CI doc build: $TRAVIS_COMMIT_MESSAGE"
  git push -u origin gh-pages
fi
