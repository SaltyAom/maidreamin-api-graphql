#! /bin/bash
clear

echo -e "\n\nRust - Juniper GraphQL\n" 
wrk http://127.0.0.1:8080/graphql -d 10 -t 1 -c 20 -s benchmark.lua 

echo -e "\nNode - Apollo GraphQL\n" 
wrk http://127.0.0.1:8081 -d 10 -t 1 -c 20 -s benchmark.lua 

echo -e "\n\n"