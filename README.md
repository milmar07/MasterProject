# MasterProject

SETUP PROCEDURE

install docker using the script below:
sudo apt-get update -y

sudo apt-get install \
    ca-certificates \
    curl \
    gnupg \
    lsb-release -y


#Add docker GPG key
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg


#Setup docker repository
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

#Install docker engine
sudo apt-get update -y
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-compose-plugin -y

#Enable docker without sudo
sudo groupadd docker
sudo usermod -aG docker $USER

#Restart VM and try docker
#newgrp docker
#docker run hello-world


Start the node-docker-setup
git clone https://github.com/iotaledger/node-docker-setup.git

cd node-docker-setup/docker

echo "COMPOSE_FILE=docker-compose.yml" >> .env
echo "ACME_EMAIL=<your email>" >> .env
echo "NODE_HOST=<your public EC2 DNS >" >> .env
echo "HTTP_PORT=9000" >> .env

sudo ./prepare_docker.sh

#Set admin as password
echo "DASHBOARD_PASSWORD=6d386723b3d573458548e20f37f1a556ec653f49e9c2c5133070126f2b7a897d" >> .env
echo "DASHBOARD_SALT=76299b26accd3d4d755be98a898476db47934ff49d63c7f2e06d337ae755df03" >> .env
echo "COMPOSE_PROFILES=monitoring,wasp" >> .env

docker compose up -d


install wasp-cli
#Install Prerequirements
sudo apt update -y
sudo apt install golang-go -y

sudo add-apt-repository ppa:ethereum/ethereum -y
sudo apt-get update -y
sudo apt-get install solc -y

sudo apt install make -y

#Install wasp
git clone https://github.com/iotaledger/wasp.git
cd wasp
git checkout v0.3.8
make install

#Add wasp-cli to path
echo "export PATH=$PATH:$(go env GOPATH)/bin" >> ~/.bashrc
source ~/.bashrc

#Initialize wasp-cli
cd ..

wasp-cli init

wasp-cli set l1.apiaddress https://api.testnet.shimmer.network
wasp-cli set l1.faucetaddress https://faucet.testnet.shimmer.network

wasp-cli set wasp.0.api http://<your public EC2 DNS>:9000/wasp/api
wasp-cli set wasp.0.nanomsg http://localhost:5550
wasp-cli set wasp.0.peering http://localhost:4000

wasp-cli request-funds
