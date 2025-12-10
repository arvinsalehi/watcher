# Use a slim Python image
FROM python:3.9-slim

# Set the working directory
WORKDIR /app

# Copy requirements and install them
COPY . .
RUN pip install Flask

# Command to run the application
CMD ["python", "app.py"]

