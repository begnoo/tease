#!/bin/bash

gnome-terminal --tab -- bash -ic "set-title storage; cd storage_service; cargo run; exec bash;"
gnome-terminal --tab -- bash -ic "set-title repo_db; cd RepoService; sudo docker-compose -f db-compose.yml up; exec bash;"
gnome-terminal --tab -- bash -ic "set-title repo_service; cd RepoService; go build; ./RepoService; exec bash;"
gnome-terminal --tab -- bash -ic "set-title user_db; cd UserService; sudo docker-compose -f db-compose.yml up; ./UserService; exec bash;"
gnome-terminal --tab -- bash -ic "set-title user_service; cd UserService; go build; ./UserService; exec bash;"
gnome-terminal --tab -- bash -ic "set-title front; cd frontend; yarn run start; exec bash;"
