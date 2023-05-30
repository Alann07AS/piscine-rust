#!/bin/bash

read cname
git add *
git commit -m "$cname"
git push