import React, { useState } from 'react';
import sr from "../assets/search-arrow.svg";

function SearchBar({addTask}) {
    const [inputValue, setInputValue] = useState("");

    const handleKeyDown = async (e) => {
        if (e.key === "Enter" && inputValue.trim() !== "") {
            await addTask(inputValue);
            setInputValue("");
        }
    }
  return (
    <div className="mt-10 mb-5 shadow-2xl shadow-gray-500">
        <div className='bg-zinc-400 w-full h-12 rounded-sm flex items-center py-5 px-2 gap-3'>
            <img src={sr} className='w-5 h-5'/>
            <input 
                type='text'
                value={inputValue}
                onChange={(e) => setInputValue(e.target.value)}
                onKeyDown={handleKeyDown}
                className="w-full focus:outline-none caret-gray-500" 
                placeholder='Add a task...'
            />
        </div>
    </div>
  )
}

export default SearchBar