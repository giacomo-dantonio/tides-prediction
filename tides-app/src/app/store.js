import { configureStore, getDefaultMiddleware } from '@reduxjs/toolkit';
import reducer from '../slices';

export default configureStore({
  reducer,
  middleware: getDefaultMiddleware({
    serializableCheck: false,
    immutableCheck: false
  })
});
