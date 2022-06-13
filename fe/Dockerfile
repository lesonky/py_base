FROM node:16 as base
WORKDIR /app

COPY ./package*.json ./
RUN npm install -registry=https://registry.npm.taobao.org

COPY . .
RUN npm run build --release


FROM nginx:1.21.6
#
WORKDIR /app
COPY --from=base  /app/dist /app/dist
