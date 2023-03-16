# gcloud-rs

Wrapper for the gcloud cli tool written in Rust.

~~Currently does nothing, hopefully soon it will do <something>~~

## How to use
* create a JSON file based off your service account
* connect to it with `gcloud auth activate-service-account <service-account>@cyderes-test.iam.gserviceaccount.com --key-file=/<path-to-json>/test.json --project=cyderes-test`
* connect to gcloud with the following command `gcloud auth application-default login`
* run the script