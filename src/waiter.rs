use crate::conditions::handle_errors;
use crate::{conditions, ElementPoller, ElementPollerTicker, ElementPredicate};
use std::time::Duration;
use stringmatch::Needle;
use thirtyfour::error::WebDriverError;
use thirtyfour::prelude::WebDriverResult;
use thirtyfour::WebElement;

#[derive(Debug, Clone)]
pub struct ElementWaiter<'a> {
    element: &'a WebElement<'a>,
    poller: ElementPoller,
    message: String,
    ignore_errors: bool,
}

impl<'a> ElementWaiter<'a> {
    fn new<S>(element: &'a WebElement<'a>, poller: ElementPoller, message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            element,
            poller,
            message: message.into(),
            ignore_errors: true,
        }
    }

    /// Use the specified ElementPoller for this ElementWaiter.
    /// This will not affect the default ElementPoller used for other waits.
    pub fn with_poller(mut self, poller: ElementPoller) -> Self {
        self.poller = poller;
        self
    }

    /// By default a waiter will ignore any errors that occur while polling for the desired
    /// condition(s). However, this behaviour can be modified so that the waiter will return
    /// early if an error is returned from thirtyfour.
    pub fn ignore_errors(mut self, ignore: bool) -> Self {
        self.ignore_errors = ignore;
        self
    }

    /// Force this ElementWaiter to wait for the specified timeout, polling once
    /// after each interval. This will override the poller for this
    /// ElementWaiter only.
    pub fn wait(self, timeout: Duration, interval: Duration) -> Self {
        self.with_poller(ElementPoller::TimeoutWithInterval(timeout, interval))
    }

    async fn run_poller(&self, conditions: Vec<ElementPredicate>) -> WebDriverResult<bool> {
        let mut ticker = ElementPollerTicker::new(self.poller.clone());
        loop {
            let mut conditions_met = true;
            for f in &conditions {
                if !f(&self.element).await? {
                    conditions_met = false;
                    break;
                }
            }

            if conditions_met {
                return Ok(true);
            }

            if !ticker.tick().await {
                return Ok(false);
            }
        }
    }

    fn timeout(self) -> WebDriverResult<()> {
        Err(WebDriverError::Timeout(self.message))
    }

    pub async fn condition(self, f: ElementPredicate) -> WebDriverResult<()> {
        match self.run_poller(vec![f]).await? {
            true => Ok(()),
            false => self.timeout(),
        }
    }

    pub async fn conditions(self, conditions: Vec<ElementPredicate>) -> WebDriverResult<()> {
        match self.run_poller(conditions).await? {
            true => Ok(()),
            false => self.timeout(),
        }
    }

    pub async fn stale(self) -> WebDriverResult<()> {
        let ignore_errors = self.ignore_errors;
        self.condition(Box::new(move |elem| {
            Box::pin(
                async move { handle_errors(elem.is_present().await.map(|x| !x), ignore_errors) },
            )
        }))
        .await
    }

    pub async fn displayed(self) -> WebDriverResult<()> {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_is_displayed(ignore_errors)).await
    }

    pub async fn not_displayed(self) -> WebDriverResult<()> {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_is_not_displayed(ignore_errors)).await
    }

    pub async fn selected(self) -> WebDriverResult<()> {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_is_selected(ignore_errors)).await
    }

    pub async fn not_selected(self) -> WebDriverResult<()> {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_is_not_selected(ignore_errors)).await
    }

    pub async fn enabled(self) -> WebDriverResult<()> {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_is_enabled(ignore_errors)).await
    }

    pub async fn not_enabled(self) -> WebDriverResult<()> {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_is_not_enabled(ignore_errors)).await
    }

    pub async fn clickable(self) -> WebDriverResult<()> {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_is_clickable(ignore_errors)).await
    }

    pub async fn not_clickable(self) -> WebDriverResult<()> {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_is_not_clickable(ignore_errors)).await
    }

    pub async fn has_class<N>(self, class_name: N) -> WebDriverResult<()>
    where
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_has_class(class_name, ignore_errors)).await
    }

    pub async fn lacks_class<N>(self, class_name: N) -> WebDriverResult<()>
    where
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_lacks_class(class_name, ignore_errors)).await
    }

    pub async fn has_text<N>(self, text: N) -> WebDriverResult<()>
    where
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_has_text(text, ignore_errors)).await
    }

    pub async fn lacks_text<N>(self, text: N) -> WebDriverResult<()>
    where
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_lacks_text(text, ignore_errors)).await
    }

    pub async fn has_value<N>(self, value: N) -> WebDriverResult<()>
    where
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_has_value(value, ignore_errors)).await
    }

    pub async fn lacks_value<N>(self, value: N) -> WebDriverResult<()>
    where
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_lacks_value(value, ignore_errors)).await
    }

    pub async fn has_attribute<S, N>(self, attribute_name: S, value: N) -> WebDriverResult<()>
    where
        S: Into<String>,
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_has_attribute(attribute_name, value, ignore_errors))
            .await
    }

    pub async fn lacks_attribute<S, N>(self, attribute_name: S, value: N) -> WebDriverResult<()>
    where
        S: Into<String>,
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_lacks_attribute(attribute_name, value, ignore_errors))
            .await
    }

    pub async fn has_attributes<S, N>(self, desired_attributes: &[(S, N)]) -> WebDriverResult<()>
    where
        S: Into<String> + Clone,
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_has_attributes(desired_attributes, ignore_errors)).await
    }

    pub async fn lacks_attributes<S, N>(self, desired_attributes: &[(S, N)]) -> WebDriverResult<()>
    where
        S: Into<String> + Clone,
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_lacks_attributes(desired_attributes, ignore_errors))
            .await
    }

    pub async fn has_property<S, N>(self, property_name: S, value: N) -> WebDriverResult<()>
    where
        S: Into<String>,
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_has_property(property_name, value, ignore_errors)).await
    }

    pub async fn lacks_property<S, N>(self, property_name: S, value: N) -> WebDriverResult<()>
    where
        S: Into<String>,
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_lacks_property(property_name, value, ignore_errors))
            .await
    }

    pub async fn has_properties<S, N>(self, desired_properties: &[(S, N)]) -> WebDriverResult<()>
    where
        S: Into<String> + Clone,
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_has_properties(desired_properties, ignore_errors)).await
    }

    pub async fn lacks_properties<S, N>(self, desired_properties: &[(S, N)]) -> WebDriverResult<()>
    where
        S: Into<String> + Clone,
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_lacks_properties(desired_properties, ignore_errors))
            .await
    }

    pub async fn has_css_property<S, N>(self, css_property_name: S, value: N) -> WebDriverResult<()>
    where
        S: Into<String>,
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_has_css_property(
            css_property_name,
            value,
            ignore_errors,
        ))
        .await
    }

    pub async fn lacks_css_property<S, N>(
        self,
        css_property_name: S,
        value: N,
    ) -> WebDriverResult<()>
    where
        S: Into<String>,
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_lacks_css_property(
            css_property_name,
            value,
            ignore_errors,
        ))
        .await
    }

    pub async fn has_css_properties<S, N>(
        self,
        desired_css_properties: &[(S, N)],
    ) -> WebDriverResult<()>
    where
        S: Into<String> + Clone,
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_has_css_properties(
            desired_css_properties,
            ignore_errors,
        ))
        .await
    }

    pub async fn lacks_css_properties<S, N>(
        self,
        desired_css_properties: &[(S, N)],
    ) -> WebDriverResult<()>
    where
        S: Into<String> + Clone,
        N: Needle + Clone + Send + Sync + 'static,
    {
        let ignore_errors = self.ignore_errors;
        self.condition(conditions::element_lacks_css_properties(
            desired_css_properties,
            ignore_errors,
        ))
        .await
    }
}

/// Trait for enabling the ElementWaiter interface.
pub trait ElementWaitable {
    fn wait_until<S>(&self, timeout_message: S) -> ElementWaiter
    where
        S: Into<String>;
}

impl ElementWaitable for WebElement<'_> {
    /// Return an ElementQuery instance for more executing powerful element queries.
    fn wait_until<S>(&self, timeout_message: S) -> ElementWaiter
    where
        S: Into<String>,
    {
        let poller: ElementPoller =
            self.session.config().get("ElementPoller").unwrap_or(ElementPoller::NoWait);
        ElementWaiter::new(&self, poller, timeout_message)
    }
}

#[cfg(test)]
/// This function checks if the public async methods implement Send. It is not intended to be executed.
async fn _test_is_send() -> WebDriverResult<()> {
    use thirtyfour::prelude::*;

    // Helper methods
    fn is_send<T: Send>() {}
    fn is_send_val<T: Send>(_val: &T) {}

    // Pre values
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).await?;
    let elem = driver.find_element(By::Css(r#"div"#)).await?;

    // ElementWaitCondition
    is_send_val(&elem.wait_until("Some error").stale());
    is_send_val(&elem.wait_until("Some error").displayed());
    is_send_val(&elem.wait_until("Some error").selected());
    is_send_val(&elem.wait_until("Some error").enabled());
    is_send_val(&elem.wait_until("Some error").condition(Box::new(|elem| {
        Box::pin(async move { elem.is_enabled().await.or(Ok(false)) })
    })));

    Ok(())
}
