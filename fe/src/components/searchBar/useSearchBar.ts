import { ref } from 'vue';

const useSearchBar = () => {
  const searchBarRef = ref();
  const reset = () => {
    searchBarRef.value && searchBarRef.value.reset();
  };

  return {
    searchBarRef,
    reset,
  };
};

export default useSearchBar;
