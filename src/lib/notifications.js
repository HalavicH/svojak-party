import Notiflix from "notiflix";

Notiflix.Notify.init({
    timeout: 4000,
    position: 'left-top',
});

export const notify = Notiflix.Notify;
