FROM node:20-alpine

WORKDIR /app

COPY package.json ./
RUN npm install

COPY . .

EXPOSE 8080

USER root

CMD ["node", "orchestrator.js"]
