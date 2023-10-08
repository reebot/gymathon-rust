document.addEventListener("DOMContentLoaded", function() {
    document.getElementById('fetchButton').addEventListener('click', fetchData);
});

async function fetchData() {
    try {
        const apiUrl = 'http://localhost:8000/activities';
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
    dataContainer.innerHTML = '';

    // Displaying Date below the Fetch Data Button
    const dateLabel = document.createElement('p');
    dateLabel.textContent = `Date: ${data[0].date}`;
    dataContainer.appendChild(dateLabel);

    const table = document.createElement('table');
    dataContainer.appendChild(table);

    const headers = ['Name + Centre Name', 'Starting Time'];
    const thead = document.createElement('thead');
    const headerRow = document.createElement('tr');
    headers.forEach(header => {
        const th = document.createElement('th');
        th.textContent = header;
        headerRow.appendChild(th);
    });
    thead.appendChild(headerRow);
    table.appendChild(thead);

    if (Array.isArray(data) && data.length > 0) {
        const tbody = document.createElement('tbody');
        table.appendChild(tbody);

        data.forEach(activity => {
            const row = document.createElement('tr');

            // Name + Centre Name column
            const nameTd = document.createElement('td');
            nameTd.textContent = `${activity.name} (${activity.centerName})`; // Corrected property name
            row.appendChild(nameTd);

            // Starting time column
            const timeTd = document.createElement('td');
            timeTd.textContent = activity.startTime; // Corrected property name
            row.appendChild(timeTd);

            tbody.appendChild(row);
        });
    } else {
        dataContainer.innerHTML = '<p>No activities available.</p>';
    }
}

function bookActivity(activityId) {
    alert(`Booking activity with ID: ${activityId}`);
}

function showInfoPopup(description) {
    const overlay = document.createElement('div');
    overlay.className = 'overlay';

    const modal = document.createElement('div');
    modal.className = 'modal';

    const closeButton = document.createElement('button');
    closeButton.textContent = 'Close';
    closeButton.onclick = () => document.body.removeChild(overlay);

    const descriptionP = document.createElement('p');
    descriptionP.textContent = description;

    modal.appendChild(descriptionP);
    modal.appendChild(closeButton);
    overlay.appendChild(modal);
    document.body.appendChild(overlay);
}
