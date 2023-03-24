#!/bin/bash
for dir in $(find . -type d -maxdepth 1); do rm ${dir}/.git -rf; done