import Notiflix from "notiflix";

Notiflix.Notify.init({
    timeout: 4000,
    position: 'left-top',
});

let allowNotifications = false;
export const notify = {
    info: (i) => {
        if (allowNotifications) {
            Notiflix.Notify.info(i)
        }
    },
    failure: (i) => {
        if (allowNotifications) {
            Notiflix.Notify.failure(i)
        }
    }
};

export function setAllowNotifications(b) {
    allowNotifications = b;
}