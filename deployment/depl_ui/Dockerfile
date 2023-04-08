FROM node:14-alpine

WORKDIR /app

COPY ui/package*.json ./
RUN npm install

COPY ui /app

EXPOSE 3000

CMD ["npm", "start"]
