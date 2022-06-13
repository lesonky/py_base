FROM nvidia/cuda:10.1-base-ubuntu18.04

#install python3.7, use aliyun as apt source
COPY ./ubuntu18.04_aliyuns_sources.list /etc/apt/sources.list
RUN apt-get update && apt install -y  build-essential zlib1g-dev libbz2-dev libncurses5-dev libgdbm-dev libnss3-dev libssl-dev libreadline-dev libffi-dev wget
RUN apt-get install -y liblzma-dev

RUN wget https://npm.taobao.org/mirrors/python/3.7.4/Python-3.7.4.tgz

RUN tar -xzvf Python-3.7.4.tgz && cd Python-3.7.4 && ./configure --prefix=/usr/local/python3.7.4 && make && make install


RUN ln -s /usr/local/python3.7.4/bin/python3.7  /usr/bin/python3 \ 
      && ln -s /usr/local/python3.7.4/bin/pip3.7 /usr/bin/pip3
RUN pip3 config set global.index-url https://pypi.doubanio.com/simple
RUN pip3 install --upgrade pip
WORKDIR /app

# install your dep packages
RUN pip3 install torch>=1.11.0
RUN pip3 install torchcam>=0.3.1
RUN pip3 install torchvision>=0.12.0

RUN pip3 install pytorch-lightning>=1.6.1
RUN pip3 install opencv-python==4.5.5.64
RUN pip3 install opencv-python-headless>=4.5.4
RUN pip3 install scikit-image==0.19.2 
RUN pip3 install scikit-learn==1.0.2
RUN pip3 install pandas>=1.4.2
RUN pip3 install albumentations>=1.1.0
RUN pip3 install dotmap>=1.3.30
RUN pip3 install pretrainedmodels>=0.7.4
RUN pip3 install efficientnet-pytorch>=0.7.1
RUN pip3 install filelock>=3.7.1
