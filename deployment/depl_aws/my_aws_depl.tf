provider "aws" {
  access_key = "YOUR ACCESS KEY HERE"
  secret_key = "YOUR SECRET KEY HERE"
  region     = "eu-west-1"
}

resource "aws_key_pair" "example" {
  key_name   = "example-keypair"
  public_key = file("~/.ssh/notes_id_rsa.pub")
}

// Security groups
resource "aws_security_group" "external_sg" {
  name_prefix = "database"

  //default port for SSH
  ingress {
    from_port = 22
    to_port   = 22
    protocol  = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port = 3000
    to_port   = 3000
    protocol  = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port = 5000
    to_port   = 5000
    protocol  = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  //To allow outbound internet traffic to install Docker
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
resource "aws_security_group" "internal_sg" {
  name_prefix = "internal"

  ingress {
    from_port = 5432
    to_port   = 5432
    protocol  = "tcp"
    security_groups = [aws_security_group.external_sg.id]
  }

  //To respond to incoming requests that were allowed by the ingress rules
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

//Boilerplate code to get the instance permission to read the S3 bucket "guillerbucket"
resource "aws_iam_role" "example" {
  name = "example-role"
  
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "ec2.amazonaws.com"
        }
      }
    ]
  })
}
resource "aws_iam_policy" "s3-read-only-policy" {
  name        = "s3-read-only-policy"
  description = "Read-only access to S3 bucket"
  policy      = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid = "S3ReadObject"
        Effect = "Allow"
        Action = [
          "s3:GetObject"
        ]
        Resource = [
          "arn:aws:s3:::guillerbucket/*"
        ]
      }
    ]
  })
}
resource "aws_iam_role_policy_attachment" "example" {
  policy_arn = aws_iam_policy.s3-read-only-policy.arn
  role       = aws_iam_role.example.name
}
resource "aws_iam_instance_profile" "allow_s3_read" {
  name = "example-instance-profile"
  role = aws_iam_role.example.id
}

resource "aws_instance" "my_instance" {
  ami           = "ami-04b8ac8af30d436d9" //(Amazon Linux 2) Needs to be an AMI from eu-west-1!
  instance_type = "t2.micro"
  key_name      = aws_key_pair.example.key_name
  vpc_security_group_ids = [
    aws_security_group.external_sg.id,
    aws_security_group.internal_sg.id
  ]
  iam_instance_profile = aws_iam_instance_profile.allow_s3_read.name

  root_block_device {
    volume_size = 29
    volume_type = "gp3"
  }

  connection {
    type     = "ssh"
    user     = "ec2-user"
    private_key = file("~/.ssh/notes_id_rsa")
    host     = self.public_ip
  }

  provisioner "remote-exec" {
    inline = [
      "sudo yum update -y",
      "sudo amazon-linux-extras install -y docker",
      "sudo service docker start",
      "sudo systemctl enable docker",
      "sudo usermod -a -G docker ec2-user",
      "sudo curl -L https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m) -o /usr/local/bin/docker-compose",
      "sudo chmod +x /usr/local/bin/docker-compose",
      "sudo chmod 666 /var/run/docker.sock",
      "aws s3 cp s3://guillerbucket/myApp/docker-compose.yaml .",
      "aws s3 cp s3://guillerbucket/myApp/postgresql.conf .",
      "docker-compose up -d"
    ]
  }
}

// Docker run the ui separately to pass the correct IP to call the server
resource "null_resource" "docker_run" {
  provisioner "remote-exec" {
    inline = [
      "docker run -d -p 3000:3000 -e REACT_APP_API_URL=http://${aws_instance.my_instance.public_ip}:5000/notes guillemaru/todoapp-react-component:latest",
    ]

    connection {
      type        = "ssh"
      user        = "ec2-user"
      host        = "${aws_instance.my_instance.public_ip}"
      private_key = file("~/.ssh/notes_id_rsa")
    }
  }

  depends_on = [aws_instance.my_instance]
}

output "public_ip" {
  value = aws_instance.my_instance.public_ip
}