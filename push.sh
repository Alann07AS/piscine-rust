#!/bin/bash

echo "Enter the commit name:"
read cname
git add *
git commit -m "$cname"
git push