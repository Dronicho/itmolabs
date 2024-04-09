import React, { useEffect, useState } from 'react';

function MessageList() {
  const [messages, setMessages] = useState([]);
  const [message, setMessage] = useState(null);
  const [target, setTarget] = useState(null);
  const email = localStorage.getItem('email');

  const handleSignOut = async () => {
    const res = await fetch('http://localhost:8080/api/auth/logout', {
      credentials: 'include',
      method: 'POST',
    });
    if (res.ok || res.status == 401) {
      localStorage.clear();
      // window.location.reload();
    }
  };

  const sendMessage = () => {
    if (message) {
      fetch('http://localhost:8080/api/messages', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify({ from: email, message, to: target }),
      }).catch(() => {});
      setMessage('');
    }
  };

  useEffect(() => {
    const fetchMessages = async () => {
      const response = await fetch('http://localhost:8080/api/messages');
      const data = await response.json();
      setMessages(data);
    };

    fetchMessages();
  }, []);

  return (
    <div className="bg-gray-50 rounded-lg dark:bg-gray-700 h-full w-full">
      <div className="flex justify-between mb-4">
        <button
          onClick={handleSignOut}
          className="bg-red-500 text-white py-2 hover:bg-red-600 font-medium rounded-lg text-sm px-5"
        >
          Logout
        </button>
      </div>
      <div className="overflow-y-auto max-h-96">
        {messages.map((message, index) => (
          <div
            key={index}
            className="bg-gray-200 p-4 rounded-lg mb-2 break-words"
          >
            <p>
              <span className="font-bold">From:</span> {message.from}
            </p>
            <p>
              <span className="font-bold">To:</span> {message.to}
            </p>
            <p>
              <span className="font-bold">Message:</span> {message.message}
            </p>
          </div>
        ))}
      </div>
      <input
        type="email"
        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-1/2 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
        placeholder="Target email"
        onInput={(e) => setTarget(e.target.value)}
      />
      <input
        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-1/2 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
        onInput={(e) => setMessage(e.target.value)}
        placeholder="Message"
      />

      <button
        className="text-white bg-primary-600 hover:bg-primary-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800"
        onClick={() => sendMessage()}
      >Send Message</button>
    </div>
  );
}

export default MessageList;
