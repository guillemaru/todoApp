[Watch a demo of the app](https://vimeo.com/815841392)


# Software required:
nodejs, Rust, PostgreSQL, Docker, Terraform, AWS CLI


# FRONTEND
-> Install nodejs
-> "ui" folder comes from creating a React project with "npx create-react-app <name>"
-> Install "axios" with "npm install axios"
-> To use Typescript in the React project:
  install the typescript package: npm install --save-dev typescript
  rename your App.js file to App.tsx.
  rename other files in your project to use the .tsx extension as needed.
  npx tsc --init (to create a tsconfig.json)
  add to tsconfig.json the following "compilerOptions" -> "jsx": "react"


# BACKEND SETUP
-> Install Rust
-> (if you want the code to recompile every time you change and save a file) cargo install cargo-watch
-> Install PostgreSQL:
  -> add C:\Program Files\PostgreSQL\15\lib and C:\Program Files\PostgreSQL\15\bin to your PATH (this is to test locally, the postgres Docker image you build will have that)
  -> edit pg_hba.conf from the PostgreSQL installation folder to have:
```
# TYPE  DATABASE        USER            ADDRESS                 METHOD

# IPv4 local connections:
host    all             all             127.0.0.1/32            trust
# IPv6 local connections:
host    all             all             ::1/128                 trust
```
  -> Only for testing in your local machine, create the database:
  psql -U postgres
  CREATE DATABASE server;
  \q
-> Setup Diesel:
  cargo install diesel_cli --no-default-features --features postgres
  diesel setup (a "migrations" folder is created after last command)
  diesel migrations generate create_notes
  (edit up.sql and down.sql from the migrations folder)
  diesel migration run

-> If you want to see contents of database:
psql -U postgres -d server -c "SELECT * FROM notes;"


# DEPLOYMENT
-> Run the containers in your machine with the top-level docker-compose.yaml file
Build the Docker images (ui, server and database) on your local machine:
$ docker-compose build
$ docker-compose up -d

-> Prepare for deployment to AWS:
  -> Either save the 3 images in a ".tar" file that you would copy into an AWS EC2 instance and run:
    -> docker save postgres todoapp-rust-api todoapp-react-component > myapp.tar
    -> Use the terraform file provisioner to copy the tar archive to the EC2 instance and the "docker load" command to load the image on the EC2 instance.
  -> Or for Windows users where copying the file to the instance is not possible:
    -> Create an account in Dockerhub (with the Docker Desktop application, for example)
    -> Push the 3 images to Dockerhub (fairly easy using Docker VSCode extension)
    -> copy a docker-compose.yaml file from your machine into an S3 bucket
    -> Get the file from S3 in your EC2 instance
    -> docker-compose up -d

-> Getting your files into an S3 bucket:
  -> Upload the docker-compose.yaml file and the postgresql.conf to an S3 bucket.
  -> Create bucket: aws s3 mb s3://mybucket
  -> Copy: 
  $ aws s3 cp docker-compose.yaml s3://guillerbucket/myApp/docker-compose.yaml
  $ aws s3 cp postgresql.conf s3://guillerbucket/myApp/postgresql.conf

-> Basic Terraform commands. In the terraform file directory:
terraform init
terraform plan
(terraform refresh)
terraform apply
terraform destroy

-> Once the AWS instance is deployed:
  -> Using "remote-exec" or SSHing into the EC2 instance terminal:
    $ aws s3 cp s3://guillerbucket/myApp/docker-compose.yaml .
    $ aws s3 cp s3://guillerbucket/myApp/postgresql.conf .
  -> Run docker-compose up -d to start the containers in the background.
  -> If you pushed your images to Docker Hub, Docker Compose will automatically pull the latest versions of the images from Docker Hub.

-> How to access the S3 bucket from the created EC2 instance:
Open the IAM console in the AWS Management Console.
Click on "Roles" in the left navigation menu, and then click on the "Create role" button.
Select "AWS service" as the trusted entity and choose "EC2" as the service that will use this role.
Click "Next: Permissions" to proceed to the permissions page.
In the "Attach permissions policies" section, click on the "Create Policy" button to create a new policy.
In the policy editor, select "JSON" and paste the following policy document
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowS3ReadAccess",
      "Effect": "Allow",
      "Action": [
        "s3:GetObject"
      ],
      "Resource": [
        "arn:aws:s3:::guillerbucket/*"
      ]
    }
  ]
}
Click on the "Review policy" button, give the policy a name, and then click on the "Create policy" button.
Go back to the "Create role" page, and refresh the list of policies. Select the policy you just created, and then click on the "Next: Tags" button.
(Optional) Add any desired tags to the role.
Click on the "Next: Review" button to review the role configuration.
Give the role a name, and then click on the "Create role" button.


