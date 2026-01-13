pub const JS: &str = r#"
const CLIENT_ID = crypto.randomUUID();

document.body.addEventListener('htmx:configRequest', function(event) {
    event.detail.headers['X-Client-Id'] = CLIENT_ID;
});

let eventSource = null;
let reconnectAttempts = 0;
const maxReconnectAttempts = 10;

function connectSSE() {
    if (eventSource) {
        eventSource.close();
    }

    eventSource = new EventSource('/events');

    eventSource.onmessage = function(event) {
        if (!event.data || event.data === 'keep-alive') {
            return;
        }

        try {
            const msg = JSON.parse(event.data);

            if (msg.client_id === CLIENT_ID) {
                return;
            }

            if (msg.type === 'reload') {
                const currentPath = window.location.pathname;
                if (currentPath !== '/') {
                    htmx.ajax('GET', currentPath, {
                        target: 'body',
                        swap: 'outerHTML'
                    });
                } else {
                    htmx.ajax('GET', '/', {
                        target: 'body',
                        swap: 'outerHTML'
                    });
                }
            }
            reconnectAttempts = 0;
        } catch (e) {
            // Handle legacy 'reload' string format or parse errors
            if (event.data === 'reload') {
                const currentPath = window.location.pathname;
                htmx.ajax('GET', currentPath, {
                    target: 'body',
                    swap: 'outerHTML'
                });
            }
        }
    };

    eventSource.onerror = function() {
        eventSource.close();
        if (reconnectAttempts < maxReconnectAttempts) {
            reconnectAttempts++;
            setTimeout(connectSSE, Math.min(1000 * Math.pow(2, reconnectAttempts), 30000));
        }
    };
}

document.addEventListener('visibilitychange', function() {
    if (!document.hidden) {
        connectSSE();
        const currentPath = window.location.pathname;
        htmx.ajax('GET', currentPath, {
            target: 'body',
            swap: 'outerHTML'
        });
    } else {
        if (eventSource) {
            eventSource.close();
        }
    }
});

connectSSE();

window.handleCheckboxClick = function(event, listId, idx) {
    event.preventDefault();
    event.stopPropagation();

    var checkbox = event.target;
    var item = checkbox.closest('.item');
    var isCompleted = item.classList.contains('completed');
    var container = document.querySelector('.container');
    var isHiding = container && container.getAttribute('data-hide-completed') === 'true';

    if (!isCompleted && isHiding) {
        item.classList.add('completed');
        checkbox.classList.add('checked');

        anime({
            targets: item,
            opacity: [1, 0.3],
            duration: 3000,
            easing: 'linear',
            complete: function() {
                anime({
                    targets: item,
                    opacity: 0,
                    translateX: -30,
                    duration: 300,
                    easing: 'easeInQuad',
                    complete: function() {
                        htmx.ajax('POST', '/list/' + listId + '/toggle/' + idx, {
                            target: 'body',
                            swap: 'outerHTML'
                        });
                    }
                });
            }
        });
    } else {
        htmx.ajax('POST', '/list/' + listId + '/toggle/' + idx, {
            target: 'body',
            swap: 'outerHTML'
        });
    }
};

window.editItem = function(element, listId, idx) {
    var itemText = element.textContent;
    var input = document.createElement('input');
    input.type = 'text';
    input.value = itemText;
    input.className = 'edit-input';
    input.style.cssText = 'background: #1f2937; border: 2px solid #2563eb; border-radius: 4px; padding: 4px 8px; color: #f3f4f6; font-size: 16px; flex: 1;';

    element.parentNode.replaceChild(input, element);
    input.focus();
    input.select();

    function finishEdit() {
        var newValue = input.value.trim();
        if (newValue && newValue !== itemText) {
            htmx.ajax('POST', '/list/' + listId + '/edit/' + idx, {
                target: 'body',
                swap: 'outerHTML',
                values: {item: newValue}
            });
        } else {
            var span = document.createElement('span');
            span.textContent = itemText;
            span.className = 'item-text';
            span.style.flex = '1';
            span.onclick = function() { window.editItem(span, listId, idx); };
            input.parentNode.replaceChild(span, input);
        }
    }

    input.addEventListener('blur', finishEdit);
    input.addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            e.preventDefault();
            finishEdit();
        }
    });
};

window.handleToggleCompleted = function(listId) {
    var completedItems = document.querySelectorAll('.item.completed');
    var menuItem = event.target.closest('.menu-item');
    var isHiding = menuItem.textContent.includes('Hide');

    document.getElementById('menu').style.display = 'none';

    if (!isHiding || completedItems.length === 0) {
        htmx.ajax('POST', '/list/' + listId + '/toggle-completed', {
            target: 'body',
            swap: 'outerHTML'
        });
        return;
    }

    anime({
        targets: completedItems,
        opacity: 0,
        translateX: -30,
        duration: 500,
        easing: 'easeInQuad',
        complete: function() {
            htmx.ajax('POST', '/list/' + listId + '/toggle-completed', {
                target: 'body',
                swap: 'outerHTML'
            });
        }
    });
};

window.handleDeleteCompleted = function(listId) {
    var completedItems = document.querySelectorAll('.item.completed');
    document.getElementById('menu').style.display = 'none';

    if (completedItems.length === 0) {
        htmx.ajax('POST', '/list/' + listId + '/delete-completed', {
            target: 'body',
            swap: 'outerHTML'
        });
        return;
    }

    anime({
        targets: completedItems,
        opacity: 0,
        translateX: -30,
        duration: 500,
        easing: 'easeInQuad',
        complete: function() {
            htmx.ajax('POST', '/list/' + listId + '/delete-completed', {
                target: 'body',
                swap: 'outerHTML'
            });
        }
    });
};

function initializeSwipes() {
    document.querySelectorAll('.item').forEach(function(itemElement) {
        if (itemElement.hammerInitialized) return;
        itemElement.hammerInitialized = true;

        var hammer = new Hammer(itemElement);
        hammer.get('pan').set({ direction: Hammer.DIRECTION_HORIZONTAL, threshold: 10 });

        var startPos = 0;
        var currentPos = 0;
        var isPanning = false;

        hammer.on('panstart', function(e) {
            isPanning = true;
            startPos = 0;
            itemElement.style.transition = 'none';
        });

        hammer.on('panmove', function(e) {
            if (!isPanning) return;
            currentPos = e.deltaX;
            itemElement.style.transform = 'translateX(' + currentPos + 'px)';
        });

        hammer.on('panend', function(e) {
            if (!isPanning) return;
            isPanning = false;

            var threshold = 100;
            var deleteUrl = itemElement.getAttribute('data-delete-url');

            if (Math.abs(currentPos) > threshold && deleteUrl) {
                anime({
                    targets: itemElement,
                    translateX: currentPos > 0 ? 300 : -300,
                    opacity: 0,
                    duration: 300,
                    easing: 'easeOutQuad',
                    complete: function() {
                        htmx.ajax('POST', deleteUrl, {
                            target: 'body',
                            swap: 'outerHTML'
                        });
                    }
                });
            } else {
                anime({
                    targets: itemElement,
                    translateX: 0,
                    duration: 300,
                    easing: 'easeOutQuad'
                });
            }

            currentPos = 0;
        });
    });
}

document.body.addEventListener('htmx:afterSwap', function(e) {
    var input = document.getElementById('add-input');
    var shouldFocus = false;

    if (e.detail && e.detail.target) {
        var targetPath = e.detail.pathInfo ? e.detail.pathInfo.requestPath : '';
        var xhr = e.detail.xhr;

        if (targetPath && targetPath.includes('/add')) {
            shouldFocus = true;
        } else if (!targetPath) {
            shouldFocus = true;
        }
    }

    if (input && shouldFocus) {
        input.focus();
    }

    setTimeout(initializeSwipes, 50);
});

document.addEventListener('DOMContentLoaded', function() {
    initializeSwipes();

    var input = document.getElementById('add-input');
    if (input) {
        input.focus();
    }
});
"#;
