# GymBooking Web App

This is a web application for booking gym activities. It allows users to view available gym activities for a specific date and book them. The application is built using a combination of Rust for the backend API and JavaScript for the frontend.

## Getting Started

To run the application locally, follow these steps:

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/gym-booking-app.git
   ```

2. Navigate to the project directory:

   ```bash
   cd gym-booking-app
   ```

3. Set up the backend:

   - Make sure you have Rust and Cargo installed on your system.
   - Navigate to the `backend` directory and run the following commands:

     ```bash
     cd backend
     cargo build
     cargo run
     ```

   This will start the backend API at `http://localhost:8000`.

4. Set up the frontend:

   - Open a new terminal window and navigate to the `frontend` directory:

     ```bash
     cd frontend
     ```

   - Start a local web server to serve the frontend:

     ```bash
     python -m http.server
     ```

   This will start a development server at `http://localhost:8001`.

5. Open your web browser and go to `http://localhost:8001` to access the GymBooking web application.

## Features

- Fetch available gym activities for a specific date.
- Display activities grouped by name and center name.
- Show starting times for each activity.
- Book activities by clicking on the starting time.
- View details about each activity.
- Responsive design for various screen sizes.

## Technologies Used

- Rust (Backend)
- JavaScript (Frontend)
- HTML and CSS (Frontend)
- SQL Database (For storing gym activity data)

## Contributing

Contributions are welcome! If you'd like to contribute to this project, please follow these guidelines:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them.
4. Push your changes to your forked repository.
5. Submit a pull request with a clear description of your changes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


