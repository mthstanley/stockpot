# Use the official Node.js runtime as the base image
FROM node:22 as build

# Set the working directory in the container
WORKDIR /app

# Copy package.json and package-lock.json to the working directory
COPY package*.json ./
COPY ui/package.json ./ui/

# Install dependencies
RUN npm install

# Copy the entire application code to the container
COPY ui ./ui/

# Build the React app for production
RUN npm run build

# Use Nginx as the production server
FROM nginx:alpine

# Copy the built React app to Nginx's web server directory
COPY --from=build /app/ui/dist /usr/share/nginx/html

COPY docker/files/nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 80 for the Nginx server
EXPOSE 80

# Start Nginx when the container runs
CMD ["nginx", "-g", "daemon off;"]
