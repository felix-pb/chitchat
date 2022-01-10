#######################
#     build stage     #
#######################
FROM node:16.13.1 AS build
WORKDIR /app
# first install the dependencies to leverage docker's build cache.
COPY package.json package-lock.json ./
RUN npm ci
# then copy the source code and build the static assets.
COPY . .
RUN npm run build

#######################
#     final stage     #
#######################
FROM nginx:1.21.5
WORKDIR /app
COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=build /app/dist /app
EXPOSE 8080
CMD [ "nginx", "-g", "daemon off;" ]
