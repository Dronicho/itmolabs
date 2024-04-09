import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

export default function HomePage() {
  const navigate = useNavigate();
  useEffect(() => {


    const fetchData = async () => {
      try {
        const response = await fetch('http://localhost:8080/api/auth/me', { credentials: 'include' });
        if (response.status === 401) {
          navigate('/login')
        } else {
          const data = await response.json();
          console.log(data);
          localStorage.setItem('email', data.email)
          localStorage.setItem('name', data.name)
          navigate('/convert')
        }
      } catch (error) {
        console.error('Error:', error);
      }
    };
    console.log('1')
    fetchData();

  }, []);

  return <div />;
}
