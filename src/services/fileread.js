// Import Tauri API
import { invoke } from '@tauri-apps/api/tauri';

// Function to read file
async function readFile(path) {
  try {
    const content = await invoke('read_file', { path });
    console.log('File content:', content);
    return content
  } catch (error) {
    console.error('Error reading file:', error);
  }
}

export default readFile