#!/bin/bash

echo "Enter the commit name:"
read cname
git add -A
git commit -m "$cname"
git push
