import local from './local';
import AppConfig from './types';

const config: AppConfig = { local }[process.env.REACT_APP_ENV || 'local']!

export { config }
