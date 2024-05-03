import Notiflix from "notiflix";

Notiflix.Notify.init({
    timeout: 4000,
});

export const notify = Notiflix.Notify;
