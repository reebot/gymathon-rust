document.addEventListener("DOMContentLoaded", function() {
	document.getElementById('fetchButton').addEventListener('click', fetchData);
});

async function fetchData() {
	try {
		const apiUrl = 'https://gymathonbe.radhikary.repl.co/api/bookings';
		const response = await fetch(apiUrl);
		const data = await response.json();
		displayData(data);
	} catch (error) {
		console.error('Error fetching data: ', error);
		alert('Failed to fetch data. Check console for error details.');
	}
}

function displayData(data) {
	const dataContainer = document.getElementById('dataContainer');
	dataContainer.innerHTML = ''; // Clear previous data

	// Create a table element
	const table = document.createElement('table');
	dataContainer.appendChild(table);

	// Add table headers
	const headers = ['Name', 'Info', 'Starting Time'];
	const thread = document.createElement('thread');
	const headerRow = document.createElement('tr');
	headers.forEach(header => {
		const th = document.createElement('th');
		th.textContent = header;
		headerRow.appendChild(th);
	});
	thread.appendChild(headerRow);
	table.appendChild(thread);

	// Check if data is an array and not empty
	if (Array.isArray(data) && data.length > 0) {
		// Add table body
		const body = document.createElement('body');
		table.appendChild(body);

		// Iterate through data and add to table
		data.forEach(booking => {
			const row = document.createElement('tr');

			// Name column
			const nameTd = document.createElement('td');
			nameTd.textContent = booking.booking.name;
			row.appendChild(nameTd);

			// Info button column
			const infoTd = document.createElement('td');
			const infoButton = document.createElement('button');
			infoButton.textContent = 'Info';
			infoButton.onclick = () => showInfoPopup(booking.booking.description);
			infoTd.appendChild(infoButton);
			row.appendChild(infoTd)

			// Starting time column
			const timeTd = document.createElement('td');
			const timeButton = document.createElement('button');
			timeButton.textContent = booking.booking.startTime;

			// Check seat availability and set border color
			if (booking.booking.bookedCount < booking.booking.classCapacity) {
				timeButton.className = 'available';
			} else {
				timeButton.className = 'not-available';
			}
			timeTd.appendChild(timeButton);
			row.appendChild(timeTd);

			// Append the row to the body
			body.appendChild(row);
		});
	} else {
		// If no data, display a message
		dataContainer.innerHTML = '<p>No bookings available.</p>';
	}
}
function showInfoPopup(description) {
	// Create overlay
	const overlay = document.createElement('div');
	overlay.className = 'overlay';

	// Create modal
	const modal = document.createElement('div');
	modal.className = 'modal';

	// Create close button
	const closeButton = document.createElement('button');
	closeButton.textContent = 'Close';
	closeButton.onclick = () => document.body.removeChild(overlay);

	// Create description paragraph
	const descriptionP = document.createElement('p');
	descriptionP.textContent = description;

	// Append elements
	modal.appendChild(descriptionP);
	modal.appendChild(closeButton);
	overlay.appendChild(modal);
	document.body.appendChild(overlay);
}
