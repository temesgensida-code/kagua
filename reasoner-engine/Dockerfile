FROM swipl:latest

WORKDIR /app

# Copy Prolog engine and rule files
COPY server.pl engine.pl ./
COPY rules/ ./rules/

# Expose default port
EXPOSE 8081

# Command to run SWI-Prolog HTTP server
CMD ["swipl", "server.pl"]
