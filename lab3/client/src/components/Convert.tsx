import { useCallback, useEffect, useRef, useState } from 'react';
import { useDropzone } from 'react-dropzone';
import { useNavigate } from 'react-router';

function ConvertVideo() {
  const dropzoneRef = useRef<HTMLInputElement>(null);
  const [file, setFile] = useState<File | null>(null);
  const [status, setStatus] = useState('');
  const [downloadLink, setDownloadLink] = useState('');
  const navigate = useNavigate()

  const onDrop = useCallback(async (acceptedFiles: File[]) => {
    const file = acceptedFiles[0];
    setFile(file);
    const formData = new FormData();
    formData.append('file', file);
    const res = await fetch('http://localhost:8080/api/upload/', {
      method: 'POST',
      body: formData,
      credentials: 'include',
    });
    if (res.ok) {
    }
  }, []);
  const { getRootProps, getInputProps, open } = useDropzone({ onDrop, noClick: true, noKeyboard: true, accept: { 'video/*': ['.mp4', '.mkv'] } });

  const handleClick = () => {
    if (dropzoneRef.current) {
      open()
    }
  };

  const handleSignOut = async () => {
    const res = await fetch('http://localhost:8080/api/auth/logout', {
      credentials: 'include',
      method: 'POST'
    });
    if (res.ok) {
      localStorage.clear()
      window.location.reload();
    }
  }

  useEffect(() => {
    const email = localStorage.getItem('email')
    if (!email) {
      navigate('/login')
    }
  }, [])


  useEffect(() => {
    if (file) {
      const es = new EventSource(
        `http://localhost:8080/api/events`, { withCredentials: true }
      );
      es.onopen = () => console.log('>>> Connection opened!');
      es.onerror = (e) => console.log('ERROR!', e);
      es.onmessage = (e) => {
        const data = JSON.parse(e.data);
        console.log(data);
        setStatus(data.status.toLowerCase());
        setDownloadLink(data['download_link']);
      };
      return () => es.close();
    }
  }, [file]);

  return (
    <>

      <h1 className="mb-4 text-4xl font-extrabold tracking-tight leading-none text-gray-900 md:text-5xl lg:text-6xl dark:text-white">Convert mp4 to webm</h1>

      <div className=" flex flex-col mb-8 lg:mb-16 space-y-4 sm:flex-row sm:justify-center sm:space-y-0 sm:space-x-4"></div>
      {(status === 'started' && (
        <div className=' h-64 bg-gray-50 rounded-lg dark:bg-gray-700 w-3/5 min-w-[300px] min-h-[240px] py-4 px-4 flex flex-col items-center justify-center mx-auto'>
          <div className="px-3 py-3 text-xs font-medium leading-none text-center text-blue-800 bg-blue-200 rounded-full animate-pulse dark:bg-blue-900 dark:text-blue-200">Processing your video...</div>

        </div>
      )) ||
        (status == 'completed' && (
          <div className=" w-3/5 min-w-[300px] min-h-[160px] px-4 mx-auto flex flex-col items-center justify-center h-64   rounded-lg bg-gray-50  dark:bg-gray-700  dark:border-gray-600 ">

            <div className="w-12 h-12 rounded-full bg-green-100 dark:bg-green-900 p-2 flex items-center justify-center mx-auto mb-3.5">
              <svg aria-hidden="true" className="w-8 h-8 text-green-500 dark:text-green-400" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd"></path></svg>
              <span className="sr-only">Success</span>
            </div>
            <p className="mb-8 text-lg font-semibold text-gray-900 dark:text-white">Video converted</p>
            <a href={downloadLink} target="_blank" download='result.webm' type="button" className="py-2 px-3 text-sm font-medium text-center text-white rounded-lg bg-primary-600 hover:bg-primary-700 focus:ring-4 focus:outline-none focus:ring-primary-300 dark:focus:ring-primary-900">
              Download
            </a>
          </div>
        )) ||
        (status === '' && (

          <div
            data-testid='dropzone'
            {...getRootProps({
              className: 'w-3/5 min-w-[300px] min-h-[160px] px-4 flex flex-col items-center justify-center mx-auto'
            })}
            onClick={handleClick}
          >
            <input {...getInputProps()} ref={dropzoneRef} className='hidden' onClick={(e) => e.stopPropagation()} />
            <div className="flex flex-col items-center justify-center w-full h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600">
              <div className="flex flex-col items-center justify-center pt-5 pb-6">
                <svg className="w-8 h-8 mb-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 16">
                  <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2" />
                </svg>
                <p className="mb-2 text-sm text-gray-500 dark:text-gray-400"><span className="font-semibold">Click to upload</span> or drag and drop</p>
                <p className="text-xs text-gray-500 dark:text-gray-400">MP4, MKV</p>
              </div>
            </div>
          </div>
        ))}

      <button onClick={handleSignOut} className="text-white bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 font-medium rounded-lg text-sm px-4 lg:px-5 py-2 lg:py-2.5 mt-12 dark:bg-primary-600 dark:hover:bg-primary-700 focus:outline-none dark:focus:ring-primary-800">Sign out</button>

    </>
  );
}

export default ConvertVideo;
